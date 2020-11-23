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
