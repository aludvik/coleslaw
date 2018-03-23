import zmq
import sys

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

def main(connect=sys.argv[1]):
    context = zmq.Context()
    chan = Channel(context, False, connect)

    for i in range(10):
        print("%d" % i)
        cmd = "put %s %s" % (i, i+1)
        chan.send(cmd.encode("utf-8"))
        cmd = "get %s" % i
        chan.send(cmd.encode("utf-8"))
        assert(chan.recv().decode("utf-8") == str(i+1))
    chan.send(b"shutdown")

if __name__ == "__main__":
    main()
