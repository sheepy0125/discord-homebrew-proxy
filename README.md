# discord-homebrew-proxy

# Building

This is just a normal Rust app. Run `cargo run` and you should be good!

# Why is this needed?

This is needed because

1. Serenity doesn't work on the 3DS
2. I don't feel like making my own Discord message listener
3. ~~My school's firewall blocks Discord.com~~

# What is sent and received?

<!-- word wrap makes this look BEAUTIFUL -->

| Sent           | Received                  | Handled                                                                                                                                                  | Purpose                                                                       |
| -------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| HELLO3DS       | HELLO3DS                  | The 3DS will finish the handshake.                                                                                                                       | A handshake for the 3DS to ensure the stream is working.                      |
| SEND\<message> | SENT                      | The proxy will send a Discord message, \<message>.                                                                                                       | The 3DS can send a message through the proxy.                                 |
| GET            | NONE or MESSAGE\<message> | The proxy will either reply with NONE if there is no message, or MESSAGE with a message if there is a message. It will display it if there is a message. | The 3DS will spam the proxy with these requests asking if there is a message. |
