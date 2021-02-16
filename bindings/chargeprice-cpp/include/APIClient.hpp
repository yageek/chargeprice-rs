#ifndef __CHARGEPRICE_API_CLIENT
#define __CHARGEPRICE_API_CLIENT
#include <string>
#include <stdint.h>
#include <functional>
#include "Vehicule.hpp"
#include <vector>
extern "C"
{
#include <chargeprice-ffi.h>
}

namespace chargeprice
{

    /**
     * @brief A cancellable element
     * 
     */

    typedef uint64_t APIClientCancellable;
    /**
     * @brief The class managing requests to the chargeprice-api
     * 
     */
    class APIClient
    {

    private:
        static void success_cb(void *context, const CAPI_Vehicule *arg, size_t length);
        static void error_cb(void *context, const CAPI_FFIError *arg);

    protected:
        CAPI_FFIClient *_client;

    public:
        /**
         * @brief Construct a new APIClient object
         * 
         * @param key The API to use
         */
        APIClient(const std::string &key);

        /**
         * @brief Destroy the APIClient object
         * 
         */
        virtual ~APIClient();

        APIClientCancellable load_vehicules(std::function<void(std::vector<Vehicule> success, void *error)> func);
    };
}; // namespace chargeprice
#endif