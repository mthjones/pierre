# pierre

![pierre](pierre.png) Bonjour!

Pierre helps your team by posting any new pull requests in Stash into your Slack channel!

## Setup

1. Create a bot for your Slack team named `pierre` and get the token
2. Determine the id of the channel you want your bot to post announcements in.
  1. To do so, go to `https://slack.com/api/channels.list?token=YOUR_TOKEN_HERE`, replacing the token with the token generated in the last step.
  2. Find the channel in the JSON with the name of the one you want
  3. Get the id of that channel (should look something like `C12345678`)
2. Create a config file at `~/.pierre_config` with the following template:
```json
  {
    "db": "<< postgres connection string >>",
    "stash": {
      "username": "<< a Stash username that has access to the projects >>",
      "password": "<< password for that Stash user >>",
      "base_url": "<< base url to Stash (including http/https) >>"
    },
    "slack": {
      "token": "<< slack bot token from step 1 >>",
      "channel": "<< channel id from step 2 >>"
    },
    "projects": [
      {
        "id": "<< Stash project ID (all uppercase – as it appears in url) >>",
        "repo": "<< Stash repository name (as it appears in url) >>"
      }
    ],
    "users": {
      "<< stash username>>": "<< slack username (including @) >>"
    }
  }
```
3. `cargo run --release`

## Attribution
Pierre's portrait lovingly provided by [robohash.org](https://robohash.org/).

## License
Pierre is distributed under the [MIT License](LICENSE).
