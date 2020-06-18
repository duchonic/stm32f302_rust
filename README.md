
<pre>
duchonic@toff:/usr/share/openocd/scripts$ sudo openocd -f board/st_nucleo_f3.cfg
duchonic@toff:~/stm32f302_rust$ arm-none-eabi-gdb -x loader.gdb target/thumbv7m-none-eabi/release/app
</pre>
