#define CATCH_CONFIG_MAIN
#include "catch.hpp"
#include <Chargeprice.hpp>
#include <iostream>

TEST_CASE("Parser initialisation")
{
    auto client = chargeprice::APIClient("some_key");

    client.load_vehicules([](std::vector<chargeprice::Vehicule> vehicule, void *error) {
        for (auto &elem : vehicule)
        {
            std::cout << "Vehicule: " << elem.id << std::endl;
        }
    });
    sleep(10);
}