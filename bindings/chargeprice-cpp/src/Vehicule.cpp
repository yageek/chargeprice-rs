#include "Vehicule.hpp"

using namespace chargeprice;
Vehicule::Vehicule(std::string rid, std::string rbrand, std::string rmanufacturer_id) : id(rid), brand(rbrand), manufacturer_id(rmanufacturer_id) {}

Vehicule::Vehicule(const Vehicule &cp) : id(cp.id), brand(cp.brand), manufacturer_id(cp.manufacturer_id) {}

Vehicule::Vehicule(const char *id, const char *brand, const char *manufacturer_id) : id(id), brand(brand), manufacturer_id(manufacturer_id)

{
}