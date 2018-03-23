import zmq

class Channel:

    def __init__(self, context, bind, endpoint):
        socket = context.socket(zmq.PAIR)
        if bind:
            socket.bind(endpoint)
        else:
            socket.connect(endpoint)

        self._socket = socket

    def recv(self):
        return self._socket.recv()

    def send(self, data):
        return self._socket.send(data)

def main(connect):
    context = zmq.Context()
    chan = Channel(context, False, connect)

    while True:
        received = chan.recv()
        print("Received '{}' from Rust".format(received.decode("utf-8")))
        if received == b"shutdown":
            return
        chan.send(b"world")

if __name__ == "__main__":
    main("ipc://test")
