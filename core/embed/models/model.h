#ifndef MODELS_MODEL_H_
#define MODELS_MODEL_H_

#include "layout_common.h"

#if defined CERBERUS_MODEL_1
#include "model_T1B1.h"
#elif defined CERBERUS_MODEL_T
#include "model_T2T1.h"
#elif defined CERBERUS_MODEL_R
#include "model_T2B1.h"
#elif defined CERBERUS_MODEL_DISC1
#include "model_D001.h"
#else
#error Unknown Cerberus model
#endif

#endif
