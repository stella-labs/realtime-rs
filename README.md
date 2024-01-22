# Supabase realtime-rs

Synchronous websocket client wrapper for Supabase realtime. WIP, API is solid as water.

## Progress

### Working so far

#### Core

 - [x] Websocket client
 - [x] Channels
 - [x] Granular Callbacks (`INSERT`, `UPDATE`, `DELETE` and `ALL` (`*`))
 - [x] Heartbeat
 - [x] Client states
 - [x] Disconnecting client
 - [x] Single threaded client
 - [x] Connection timeout + retry
 - [x] Configurable reconnect max attempts
 - [x] Auto reconnect
 - [x] Configurable client-side message throttling
 - [x] TLS websockets
 - [x] Docs
    > - [x] Make `pub(crate)` anything that doesn't need to be user facing

#### Channels

 - [x] Broadcast
   > Very basic implementation, so far untested across different devices
 - [x] Gracefully disconnecting Channels
   > more work and testing needed here
 - [x] Channel states
 - [x] Client `set_auth` + cascade through channels
   > Untested cos no GoTrue, or supabase Auth, or whatever it's calling itself nowadays. May look into remedying this lack of auth once realtime is done
 - [x] Presence
 - [x] Blocking `.subscribe()`
   > currently implemented on the client due to my skill issues with the borrow checker. plan to move it to channel once im good at coding

#### Extra

 - [x] Middleware
   > Saw the js client lib offering overrides for system functions and such, I figure middlewares for received messages can fill this gap
   > May be useless
 - [x] Placeholder function for email/password auth (immediately deprecated in favour of gotrue-rs or auth-rs or whatever it will be called.)
 - [x] Builder patterns for client and channel
 - [x] Example: CLI broadcast chatroom with active user list (accessed with a `/online` command)
 - [x] Example: Pull data out of closure and modify state in a higher scope
   > `event_counter` in `examples/cdc.rs`, `alias` in `examples/chatroom.rs`. Uses `Rc<RefCell<_>>`.

### TODOs

 - [ ] Lock down a clean API
 - [ ] Anything else I can find to do before writing tests
 - [ ] Tests
    > Many cases should be handled with docs code examples
 - [ ] Async client
 - [ ] Encode / Decode
 - [ ] Client ref
 - [ ] REST channel sending
 - [ ] Remove unused `derive`s

 #### Async

 - [ ] Client
 - [ ] Channel
 - [ ] Presence

 #### Examples

 - [ ] Example: Act on system messages && test what happens when we ignore them? Need to handle couldn't subscribe errors.

 #### Middleware

 - [ ] Middleware ordering
 - [ ] Middleware example (?) try using current API see if middleware needed
 - [ ] Middleware filtering by `MessageEvent`

# Contributing

Once I've filled the role that other realtime clients do I'll be open to extra contribution, in the mean time it's all duct tape and brute force so suggestions and PRs, while welcomed, may not be satisfactorily implemented.

# LICENSE

MIT / Apache 2, you know the drill it's a Rust project.
