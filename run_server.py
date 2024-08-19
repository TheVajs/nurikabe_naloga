# from http.server import SimpleHTTPRequestHandler
from http import server


class MyHTTPRequestHandler(server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_my_headers()

        server.SimpleHTTPRequestHandler.end_headers(self)

    def send_my_headers(self):
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")


KEEP_RUNNING = True


def keep_running():
    return KEEP_RUNNING


# def run():
# Handler = MyHTTPRequestHandler()
# Handler.extensions_map = {"": "text/plain"}
# Handler.extensions_map = {".js": "application/javascript"}

# print("Runing on: http://127.0.0.1:8080")

# with TCPServer(("127.0.0.1", 8080), Handler) as httpd:
#     try:
#         while keep_running():
#             httpd.serve_forever()
#     except KeyboardInterrupt:
#         pass
#     finally:
#         #
#         print("\nStoped!")
#         httpd.socket.close()


# if __name__ == "__main__":
#     run()

if __name__ == "__main__":
    server.test(HandlerClass=MyHTTPRequestHandler)
