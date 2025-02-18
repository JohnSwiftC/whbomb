# whbomb

Open source command-line tool to spam and then destroy malicious Discord webhooks.

# Build

Build with the `cargo build --release` command.

# Usage

whbomb \<WebHook URL\> \<Message\> \<Count\>

> **Example**: whbomb https://discord.com/api/webhooks/id/token "Example Message" 10

> Sends the message "Example Message" to the webhook link 10 times, and then deletes the webhook.

# Warning

I am not responsible for improper usage of this software. This is intended to only be used on machines that you have the permission to manipulate. Educational purposes only.
