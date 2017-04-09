use reqwest;
use serde::Serialize;
use serde_json;

use std::ffi::OsStr;
use std::io::Write;
use std::marker::PhantomData;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

pub trait Notifier {
    type Item;
    type Error;

    fn notify(&self, item: Self::Item) -> Result<(), Self::Error>;
}

#[derive(Clone)]
pub struct Sink<'a, T: 'a>(PhantomData<&'a T>);

impl<'a, T: 'a> Notifier for Sink<'a, T> {
    type Item = &'a T;
    type Error = ();

    fn notify(&self, _: Self::Item) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Clone)]
pub struct HttpNotifier<'a, T: 'a> {
    client: Arc<reqwest::Client>,
    endpoint: String,
    _ty: PhantomData<&'a T>
}

impl<'a, T: 'a> HttpNotifier<'a, T> {
    pub fn new<S: Into<String>>(client: Arc<reqwest::Client>, endpoint: S) -> Self {
        HttpNotifier {
            client: client,
            endpoint: endpoint.into(),
            _ty: PhantomData
        }
    }
}

impl<'a, T: 'a + Serialize> Notifier for HttpNotifier<'a, T> {
    type Item = &'a T;
    type Error = reqwest::Error;

    fn notify(&self, item: Self::Item) -> Result<(), Self::Error> {
        self.client.post(&self.endpoint).json(item).send().map(|_| ())
    }
}

#[allow(dead_code)]
pub struct ProcessNotifier<'a, T: 'a> {
    child: Mutex<Child>,
    _ty: PhantomData<&'a T>
}

#[allow(dead_code)]
impl<'a, T: 'a> ProcessNotifier<'a, T> {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        ProcessNotifier {
            child: Mutex::new(Command::new(program).stdin(Stdio::piped()).spawn().unwrap()),
            _ty: PhantomData
        }
    }
}

impl<'a, T: 'a + Serialize> Notifier for ProcessNotifier<'a, T> {
    type Item = &'a T;
    type Error = ();

    fn notify(&self, item: Self::Item) -> Result<(), Self::Error> {
        let input = serde_json::to_string(item).unwrap();

        let mut child = self.child.lock().unwrap();
        if let Some(ref mut stdin) = (*child).stdin {
            stdin.write_all(input.as_bytes()).unwrap();
            stdin.write_all(b"\n").unwrap();
            stdin.flush().unwrap();
        }

        Ok(())
    }
}

impl<'a, T: 'a> Drop for ProcessNotifier<'a, T> {
    fn drop(&mut self) {
        let mut child = self.child.lock().unwrap();
        let _ = (*child).kill();
    }
}