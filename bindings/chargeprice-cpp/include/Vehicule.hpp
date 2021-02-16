#ifndef _VEHICULE_HPP
#define _VEHICULE_HPP

#include <string>
namespace chargeprice
{
    struct Vehicule
    {
        friend class APIClient;

        const std::string id;
        const std::string brand;
        const std::string manufacturer_id;

        Vehicule(const Vehicule &cp);
        Vehicule(std::string id = "", std::string brand = "", std::string manufacturer_id = "");

    private:
        Vehicule(const char *id, const char *brand, const char *manufacturer_id);
    };
}; // namespace chargeprice

#endif