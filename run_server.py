from http.server import SimpleHTTPRequestHandler
from socketserver import TCPServer

KEEP_RUNNING = True


def keep_running():
    return KEEP_RUNNING


def run():
    Handler = SimpleHTTPRequestHandler
    Handler.extensions_map = {"": "text/plain"}
    Handler.extensions_map = {".js": "application/javascript"}

    print("Runing on: http://127.0.0.1:8080")

    with TCPServer(("127.0.0.1", 8080), Handler) as httpd:
        try:
            while keep_running():
                httpd.serve_forever()
        except KeyboardInterrupt:
            pass
        finally:
            #
            print("\nStoped!")
            httpd.socket.close()


if __name__ == "__main__":
    run()
