# Parametric Equalizer and Biqad Filter coefficient calculator for the max98090 audio codec
This is a calculator to convert the parameters of a peaking filter into IIR coefficients, in the fixed-point format that the max98090 audio codec requires.


~~I have no idea if I'm doing this right, or if it will even work at all.~~ It's working, but it's likely that I'm still doing something completely wrong. Not for use in production!

# How to apply the coefficients to the chip
By default, this script is hard-coded to output an equalizer profile that works well on my Asus C201 (which uses this audio codec.) For now just edit the code to meet your needs. A better way to generate custom profiles is probably coming soon.

First, make sure that the equalizer is actually enabled:
`tinymix set "Digital EQ 3 Band Switch" 1`

Then just copy and paste the output of the program to the end of this command:
`tinymix set "EQ Coefficients" (your_output_here)`

- The datasheet for this audio codec implies that the coefficients have to be set before the chip is fully initialized. However doing this on the fly does seem to work fine.

# No sound after applying coefficients
You probably applied a bad coefficient value. Just apply a working set of coefficients or run the following command to turn off the equalizer:
`tinymix set "Digital EQ 3 Band Switch" 0`
