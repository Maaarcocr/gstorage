#pragma once
#include "rust/cxx.h"
#include <memory>
#include <istream>

class Stream {
public:
    class impl;
    Stream(std::shared_ptr<impl> stream_impl);
    int read(char* buf, int n);
private:
    std::shared_ptr<impl> stream_impl;
};

class Client {
    class impl;
    std::shared_ptr<impl> client_impl;
public:
    Client();
    std::unique_ptr<Stream> read_object(std::string const& bucket, std::string const& object);
};

std::unique_ptr<Client> new_client();
