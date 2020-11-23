## TODO
 * Send a simple DHCP request and monitoring on reply.
 * Use nispor to to set ip, route.
 * Modify `/etc/resolv.conf` directly.
 * Support runtime config via `start eth1 option_a=value_b`.
 * Support config file.

## Good to have
 * Timeout support of processing socket IPC.
 * Permission: every can query, root can write.
 * Allow environment variable overriding the socket path
 * Don't use `all` feature for tokio dependency.
