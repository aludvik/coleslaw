This model uses ZMQ pair sockets with the ipc transport layer to communicate
between a Rust and Python process. The Rust process spawns a Python child
process which it has to keep track of and wait for. It then sends some messages
back and forth between the socket.

An alternative to PAIR would be REQ/REP, which would enforce strict
request/reply. This would be useful because it would enforce sending a response
to every request in an RPC model, but is less useful if the connection is used
as more of a queue/channel than an RPC medium.
