# IO Preprocessor Mode

This code allows the PIC to function as an IO Preprocessor of sorts, leaving the heavy lifting up to the much more capable STM32.
The communication protocol uses SPI and is as follows:

First Byte: (Sent by PIC)
0x00 - PIC is requesting data
0x01 - PIC is sending data

Second Byte:
PIC Send Mode: (Sent by PIC)
0x01 - Key Pressed

STM Send Mode: (Sent by STM)
0x00 - Not Ready
0x01 - No Data
0x02 - Display Command

Third byte:
PIC Send Mode: (Sent by PIC)
value - the key number (0-41)

STM Send Mode: (Sent by STM)
2nd byte was "Not Ready": Resend 2nd Byte, potentially ready this time
2nd Byte was "No Data": No third byte in this case
2nd Byte was "Display Command": bit 0: display register select, bit 1: display read/write (Always 0)

4th byte: Only send if STM is sending a display command:
Display command byte

