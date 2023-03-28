#include "client.hpp"
#include "google/cloud/storage/client.h"
#include "google/cloud/storage/grpc_plugin.h"

class Client::impl {
public:
    impl();
    friend Client;
private:
    google::cloud::storage::Client client;
};

class Stream::impl {
public:
    impl(std::shared_ptr<google::cloud::storage::ObjectReadStream> stream);
    friend Stream;
private:
    std::shared_ptr<google::cloud::storage::ObjectReadStream> stream;
};

Client::impl::impl() : client(google::cloud::storage_experimental::DefaultGrpcClient()) {}
Client::Client() : client_impl(new class Client::impl) {}
Stream::impl::impl(std::shared_ptr<google::cloud::storage::ObjectReadStream> stream) : stream(stream) {}
Stream::Stream(std::shared_ptr<impl> stream_impl) : stream_impl(stream_impl) {}

int Stream::read(char* buf, int n) {
    stream_impl->stream->read(buf, n);
    return stream_impl->stream->gcount();
}

std::unique_ptr<Stream> Client::read_object(std::string const& bucket, std::string const& object) {
    auto object_stream = std::make_shared<google::cloud::storage::ObjectReadStream>(client_impl->client.ReadObject(bucket, object));
    auto stream_impl = Stream::impl(object_stream);
    auto stream = Stream(std::make_shared<Stream::impl>(stream_impl));

    return std::make_unique<Stream>(stream);
}

std::unique_ptr<Client> new_client() {
  return std::make_unique<Client>();
}

