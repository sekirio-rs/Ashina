#include <arpa/inet.h>
#include <fcntl.h>
#include <stdio.h>

#include <iostream>
#include <thread>
#include <vector>

#include "kuro.h"

#define QD 1024
#define BACKLOG 10
#define BUF_LEN 1024
#define CORES 32

thread_local std::vector<Task<int>> TASKS;

Task<int> co_handle(std::shared_ptr<io_uring>& handle, TcpStream* stream_) {
  void* buf;
  char resp[BUF_LEN];

  if (posix_memalign(&buf, BUF_LEN, BUF_LEN)) co_return 1;

  co_await stream_->async_recv(handle, buf, BUF_LEN);

  int n = sprintf(resp, "HTTP/1.1 200 Ok\r\nServer: Ashina\r\n\r\n%s", (char*)buf);

  co_await stream_->async_send(handle, resp, n);

  delete stream_;
  co_return 0;
}

Task<int> co_serve(std::shared_ptr<io_uring>& handle) {
  TcpListener listener = TcpListener();

  listener.set_reuseaddr(true);
  listener.set_reuseport(true);
  listener.bind_socket("0.0.0.0", htons(3344));

  listener.listen_socket(BACKLOG);
  while (1) {
    auto stream_ = new TcpStream();

    co_await listener.async_accept(handle, stream_);

    TASKS.push_back(co_handle(handle, stream_));
  }

  co_return 0;
}

int main() {
  std::vector<std::thread> threads;

  for (int i = 0; i < CORES; i++) {
    std::thread h([] {
      struct io_uring ring;

      if (io_uring_queue_init(QD, &ring, 0) < 0) {
        std::cout << "io_uring_queue_init error" << std::endl;
        exit(-1);
      }

      std::shared_ptr<io_uring> handle = std::make_shared<io_uring>(ring);

      TASKS.push_back(co_serve(handle));

      async_execute(handle);
    });

    threads.push_back(std::move(h));
  }

  for (std::thread& h : threads) {
    if (h.joinable()) h.join();
  }

  return 0;
}
