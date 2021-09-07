#include "cxx_tutorial/include/blobstore.h"
#include "cxx_tutorial/src/main.rs.h"
#include <iostream>
#include <string>

BlobstoreClient::BlobstoreClient() {}

uint64_t BlobstoreClient::put(MultiBuf &buf) const {
    std::string contents;
    while (true) {
        auto chunk = next_chunk(buf);
        if (chunk.size() == 0) {
            break;
        }
        contents.append(reinterpt_cast<const char*>(chunk.data()), chunk.size());
    }
    auto blobid = std::hash<std::string>{}(contetns);
    return blobid;
}

std::unique_ptr<BlobstoreClient> new_blobstore_client()
{
    return std::unique_ptr<BlobstoreClient>(new BlobstoreClient());
}
void print()
{
    using namespace std::string_literals;
    std::cout << "はろー"s << std::endl;
}
