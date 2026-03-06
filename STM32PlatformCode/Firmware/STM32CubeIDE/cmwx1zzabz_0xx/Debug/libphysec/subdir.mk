################################################################################
# Automatically-generated file. Do not edit!
# Toolchain: GNU Tools for STM32 (13.3.rel1)
################################################################################

# Add inputs and outputs from these tool invocations to the build variables 
C_SRCS += \
../libphysec/acquisition.c \
../libphysec/packet.c \
../libphysec/pre_processing.c \
../libphysec/privacy_amplification.c \
../libphysec/quantization.c \
../libphysec/types.c \
../libphysec/utils.c

OBJS += \
./libphysec/acquisition.o \
./libphysec/packet.o \
./libphysec/pre_processing.o \
./libphysec/privacy_amplification.o \
./libphysec/quantization.o \
./libphysec/types.o \
./libphysec/utils.o 

C_DEPS += \
./libphysec/acquisition.d \
./libphysec/packet.d \
./libphysec/pre_processing.d \
./libphysec/privacy_amplification.d \
./libphysec/quantization.d \
./libphysec/types.d \
./libphysec/utils.d 


# Each subdirectory must supply rules for building sources it contributes
libphysec/%.o libphysec/%.su libphysec/%.cyclo: ../libphysec/%.c libphysec/subdir.mk
	arm-none-eabi-gcc "$<" -mcpu=cortex-m0plus -std=gnu11 -g3 -DSTM32L072xx -DCMWX1ZZABZ0XX -c -I../../../SubGHz_Phy/App -I../../../SubGHz_Phy/Target -I../../../Core/Inc -I../../../../Utilities/misc -I../../../../Utilities/timer -I../../../../Utilities/trace/adv_trace -I../../../../Utilities/lpm/tiny_lpm -I../../../../Utilities/sequencer -I../../../../Drivers/BSP/B-L072Z-LRWAN1 -I../../../../Drivers/BSP/CMWX1ZZABZ_0xx -I../../../../Drivers/STM32L0xx_HAL_Driver/Inc -I../../../../Drivers/CMSIS/Device/ST/STM32L0xx/Include -I../../../../Drivers/CMSIS/Include -I../../../../Middlewares/Third_Party/SubGHz_Phy -I../../../../Middlewares/Third_Party/SubGHz_Phy/sx1276 -I../../../SubGHz_Phy/App/libphysec -Os -ffunction-sections -Wall -fstack-usage -fcyclomatic-complexity -MMD -MP -MF"$(@:%.o=%.d)" -MT"$@"  -mfloat-abi=soft -mthumb -o "$@"

clean: clean-libphysec

clean-libphysec:
	-$(RM) ./libphysec/*.cyclo ./libphysec/*.d ./libphysec/*.o ./libphysec/*.su

.PHONY: clean-libphysec

