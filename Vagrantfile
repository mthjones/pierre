# -*- mode: ruby -*-
# vi: set ft=ruby :

# All Vagrant configuration is done below. The "2" in Vagrant.configure
# configures the configuration version (we support older styles for
# backwards compatibility). Please don't change it unless you know what
# you're doing.
Vagrant.configure(2) do |config|
  config.vm.box = "ubuntu/trusty64"

  # config.vm.network "forwarded_port", guest: 80, host: 8080

  # config.vm.network "private_network", ip: "192.168.33.10"

  # config.vm.network "public_network"
  config.vm.provider "virtualbox" do |v|
	v.memory = 2048
	v.cpus = 2
  end

  config.vm.provision "shell", inline: <<-SHELL
    sudo apt-get -y update
	sudo apt-get -y install libssl-dev
	sudo apt-get -y install postgresql postgresql-contrib
    curl -sSf https://static.rust-lang.org/rustup.sh | sh
	cargo install cargo-check
  SHELL
end
