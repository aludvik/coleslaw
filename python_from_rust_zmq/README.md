This model uses ZMQ pair sockets with the ipc transport layer to communicate
between a Rust and Python process. The Rust process spawns a Python child
process which it has to keep track of and wait for. It then sends some messages
back and forth between the socket.

An alternative to PAIR would be REQ/REP, which would enforce strict
request/reply. This would be useful because it would enforce sending a response
to every request in an RPC model, but is less useful if the connection is used
as more of a queue/channel than an RPC medium.

There are a few downsides to this method. First, there is an extra cost in
converting requests and replies into data that can be sent over the wire.
Second, you have to handle synchronization between the main process and the sub
process. Third, you have to carefully handle subprocesses to ensure you don't
leave any orphans behind. Fourth, debugging is hard because if the subprocess
dies for some reason and the main process is blocked, it may never learn that
the subprocess died and you may never see the output from the failure. Finally,
when getting this to work, I found that you have to be careful to cleanup the
ipc:// socket or else use a random identifier between runs to avoid problems.
