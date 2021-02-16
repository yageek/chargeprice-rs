#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * An FFI wrapper for the APIClient
 */
typedef struct CAPI_FFIClient CAPI_FFIClient;

/**
 * A simple wrapper for cancellable
 */
typedef uint64_t CAPI_Cancellable;

typedef struct
{
  const char *id;
  const char *brand;
  const char *manufacturer_id;
} CAPI_Vehicule;

typedef struct
{
  int32_t code;
  const char *message;
} CAPI_FFIError;

typedef struct
{
  void *context;
  void (*on_success)(void *context, const CAPI_Vehicule *arg, size_t length);
  void (*on_error)(void *context, const CAPI_FFIError *arg);
} CAPI_ArrayCallback_Vehicule__FFIError;

CAPI_FFIClient *chargeprice_create_api_client(const char *key, const char *flavor);

void chargeprice_free_api_client(CAPI_FFIClient *client);

void chargeprice_cancel(CAPI_FFIClient *client, CAPI_Cancellable cancellable);

CAPI_Cancellable chargeprice_get_vehicles(CAPI_FFIClient *client,
                                          CAPI_ArrayCallback_Vehicule__FFIError callback);

void chargeprice_init_log(void);
