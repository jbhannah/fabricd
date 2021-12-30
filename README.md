# fabricd

A wrapper service for configuring, updating, and running a [Fabric][] server.

## Features/Roadmap

- [x] Install the Fabric server launcher and download the Minecraft server for
      a configured version of Minecraft
- [ ] Manage installing and updating mods based on compatibility with the
      installed versions of Fabric and Minecraft
- [ ] Manage server and mod configuration from a single configuration file
  - [x] Server version
  - [ ] Server properties
  - [ ] Ops/whitelist
  - [ ] Mod configuration
- [ ] Launch and monitor the server as a child process
- [ ] Forward server logging to stdout
- [ ] Forward keyboard input to server stdin
- [ ] Gracefully stop the server with the `stop` command in response to system
      signals

[fabric]: https://fabricmc.net
