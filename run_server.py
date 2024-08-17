from http.server import SimpleHTTPRequestHandler
from socketserver import TCPServer


def run():
    Handler = SimpleHTTPRequestHandler
    Handler.extensions_map = {"": "text/plain"}
    Handler.extensions_map = {".js": "application/x-javascript"}

    print("Runing on: http://127.0.0.1:8000")

    with TCPServer(("127.0.0.1", 8000), Handler) as httpd:
        httpd.serve_forever()


if __name__ == "__main__":
    run()
