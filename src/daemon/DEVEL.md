<!-- vim-markdown-toc GFM -->

* [IPC Design](#ipc-design)
* [Thread design](#thread-design)
    * [`MozimThreadsManager`](#mozimthreadsmanager)
    * [`MozimThreadManager`](#mozimthreadmanager)
    * [`MozimDhcpManager`](#mozimdhcpmanager)
    * [`MozimDhcpWorker`](#mozimdhcpworker)

<!-- vim-markdown-toc -->

## IPC Design

The daemon will use unix stream socket `/tmp/mozim_socket` to communicate with
CLI or API bindings.

The CLI can use `ipc_connect()` and `ipc_exec()` to execute a command on
daemon, the daemon will reply with serialized `MozimResult`.

TODO: need more detail on the command format:

The command are:
 * `ping`                -> reply `pong` as `String`
 * `start <iface_name>`  -> reply `DhcpStatus`
 * `stop <iface_name>`   -> reply `DhcpStatus`
 * `query <iface_name>`  -> reply `DhcpStatus`
 * `dump`                -> reply `Vec<DhcpStatus>`

## Thread design

 * `threads_manager.rs: MozimThreadsManager`
 * `thread_manager.rs: MozimThreadManager`
 * `dhcp_manager.rs: MozimDhcpManager`
 * `dhcp_worker.rs: MozimDhcpWorker`

### `MozimThreadsManager`

Maintaining a `HashMap` for all the `MozimThreadManager` using interface name
as key.

Providing:
 * `new()`
 * `start_dhcp(iface_name)`
 * `query_dhcp(iface_name)`
 * `stop_dhcp(iface_name)`
 * `query_all()`

### `MozimThreadManager`

Represent the DHCP task of certain interface in the main thread and
provide communication sync channel the `MozimDhcpManager` who is running in
child thread.

Providing:

 * `new()`
 * `start_dhcp()`
 * `query_dhcp()`
 * `stop_dhcp()`

### `MozimDhcpManager`

Represent the DHCP task of certain interface in child thread.
The `MozimDhcpManager` will listen on sync channel, waiting:

 * `MozimDhcpCmd` from `MozimThreadManager` for command

 * `MozimDhcpCmd::StateUpdate<DhcpStatus>` from `MozimDhcpWorker` for
   status update which will be used to reply the `QueryDhcp` command.

Providing:
 * `run()`


### `MozimDhcpWorker`

TODO
