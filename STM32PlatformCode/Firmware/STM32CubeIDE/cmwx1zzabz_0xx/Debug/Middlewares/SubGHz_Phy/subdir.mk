################################################################################
# Automatically-generated file. Do not edit!
# Toolchain: GNU Tools for STM32 (13.3.rel1)
################################################################################

# Add inputs and outputs from these tool invocations to the build variables 
C_SRCS += \
$(PROJECT_ROOT)/STM32PlatformCode/Middlewares/Third_Party/SubGHz_Phy/sx1276/sx1276.c 

OBJS += \
./Middlewares/SubGHz_Phy/sx1276.o 

C_DEPS += \
./Middlewares/SubGHz_Phy/sx1276.d 


# Each subdirectory must supply rules for building sources it contributes
Middlewares/SubGHz_Phy/sx1276.o: $(PROJECT_ROOT)/STM32PlatformCode/Middlewares/Third_Party/SubGHz_Phy/sx1276/sx1276.c Middlewares/SubGHz_Phy/subdir.mk
	arm-none-eabi-gcc "$<" -mcpu=cortex-m0plus -std=gnu11 -g3 -DSTM32L072xx -DCMWX1ZZABZ0XX -c -I../../../SubGHz_Phy/App -I../../../SubGHz_Phy/Target -I../../../Core/Inc -I../../../../Utilities/misc -I../../../../Utilities/timer -I../../../../Utilities/trace/adv_trace -I../../../../Utilities/lpm/tiny_lpm -I../../../../Utilities/sequencer -I../../../../Drivers/BSP/B-L072Z-LRWAN1 -I../../../../Drivers/BSP/CMWX1ZZABZ_0xx -I../../../../Drivers/STM32L0xx_HAL_Driver/Inc -I../../../../Drivers/CMSIS/Device/ST/STM32L0xx/Include -I../../../../Drivers/CMSIS/Include -I../../../../Middlewares/Third_Party/SubGHz_Phy -I../../../../Middlewares/Third_Party/SubGHz_Phy/sx1276 -I../../../SubGHz_Phy/App/libphysec -Os -ffunction-sections -Wall -fstack-usage -fcyclomatic-complexity -MMD -MP -MF"$(@:%.o=%.d)" -MT"$@"  -mfloat-abi=soft -mthumb -o "$@"

clean: clean-Middlewares-2f-SubGHz_Phy

clean-Middlewares-2f-SubGHz_Phy:
	-$(RM) ./Middlewares/SubGHz_Phy/sx1276.cyclo ./Middlewares/SubGHz_Phy/sx1276.d ./Middlewares/SubGHz_Phy/sx1276.o ./Middlewares/SubGHz_Phy/sx1276.su

.PHONY: clean-Middlewares-2f-SubGHz_Phy

