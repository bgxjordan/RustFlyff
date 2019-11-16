# RustFlyff
About
---
Flyff v6 emulator written in Rust. Its intent is to be a lightweight, extensible and secure solution that runs as a distributed architecture.

Legal
---
This project is licenced under the GPLv3. Here's the important points to that:
* You are free to use and distribute this software, but you cannot sell it.
* You are free to modify the source code, but your changes must be published.
* If you use ("borrow") code from this project, the GPL requires that your project also comply with the GPL requirements.

This software provides no assurances or warranties. The author(s) and contributor(s) are not liable. As with any software, if you intend to expose this to the internet, you *must* practice good security. 

Software Requirements
---
* MySQL 5.5 or 5.7 (or the MariaDB equivalent) for now. Postgres and NoSQL options are being considered.
* RabbitMQ - Communication between servers is *entirely* based on a central queue system.

System Requirements
---
This assumes that you are running in the simplest configuration (1 of each server type, plus the required software is installed and running).

* At least a dual core processor (4 or more cores recommended)
* Minimum 2GB RAM available to the applications. This is to give MySQL and RabbitMQ breathing room as well as your servers.

Note: You WILL need more RAM if you add additional clusters or worlds.

Servers
---
Start the servers in the following order. Reverse the order for shutdown.

1. Database Server
2. Login Server
3. Cluster Server
4. World Server

Compiling
---
You will need a modern (2019 or later) rust toolchain. Simply run `cargo build` from the project's root directory. 

Feedback
---
As with any open source project, constructive feedback is welcome and encouraged! 

### Pull Requests
Are you a developer and would like to contribute? If it's a small change, open a pull request. For larger or more invasive changes, open an issue so that we can discuss.

### Testing
Not a developer but want to contribute? There's never a shortage of things to test.