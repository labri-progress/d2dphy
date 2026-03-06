################################################################################
# Automatically-generated file. Do not edit! Just Kidding I literally rewrote every fkn "automatically generated" makefile oupsi
# Toolchain: GNU Tools for STM32 (13.3.rel1)
################################################################################

# Add inputs and outputs from these tool invocations to the build variables 
C_SRCS += \
../libphysec/reconciliation/reconciliation.c \
../libphysec/reconciliation/fuzzy_extractor_sample_then_lock.c \

OBJS += \
./libphysec/reconciliation/reconciliation.o \
./libphysec/reconciliation/fuzzy_extractor_sample_then_lock.o \

C_DEPS += \
./libphysec/reconciliation/reconciliation.d \
./libphysec/reconciliation/fuzzy_extractor_sample_then_lock.d \

# Each subdirectory must supply rules for building sources it contributes
libphysec/reconciliation/%.o libphysec/reconciliation/%.su libphysec/reconciliation/%.cyclo: ../libphysec/reconciliation/%.c libphysec/reconciliation/subdir.mk
	arm-none-eabi-gcc "$<" -mcpu=cortex-m0plus -std=gnu11 -O2 -DNDEBUG -DSTM32L072xx -DCMWX1ZZABZ0XX -c -I../../../SubGHz_Phy/App -I../../../SubGHz_Phy/Target -I../../../Core/Inc -I../../../../Utilities/misc -I../../../../Utilities/timer -I../../../../Utilities/trace/adv_trace -I../../../../Utilities/lpm/tiny_lpm -I../../../../Utilities/sequencer -I../../../../Drivers/BSP/B-L072Z-LRWAN1 -I../../../../Drivers/BSP/CMWX1ZZABZ_0xx -I../../../../Drivers/STM32L0xx_HAL_Driver/Inc -I../../../../Drivers/CMSIS/Device/ST/STM32L0xx/Include -I../../../../Drivers/CMSIS/Include -I../../../../Middlewares/Third_Party/SubGHz_Phy -I../../../../Middlewares/Third_Party/SubGHz_Phy/sx1276 -I../../../SubGHz_Phy/App/libphysec -Os -ffunction-sections -Wall -fstack-usage -fcyclomatic-complexity -MMD -MP -MF"$(@:%.o=%.d)" -MT"$@"  -mfloat-abi=soft -mthumb -o "$@"

clean: clean-libphysec

clean-libphysec-reconciliation:
	-$(RM) ./libphysec/reconciliation/*.cyclo ./libphysec/reconciliation/*.d ./libphysec/reconciliation/*.o ./libphysec/reconciliation/*.su

.PHONY: clean-libphysec-reconciliation

