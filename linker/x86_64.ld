# SPDX-License-Identifier: GPL-3.0-or-later
OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(x86_64)
ENTRY(native_x86_64_start)

load_addr = 0xffffffff80000000;

PHDRS
{
    headers PT_PHDR PHDRS;
    text    PT_LOAD FILEHDR PHDRS;
    rodata  PT_LOAD;
    data    PT_LOAD;
    dynamic PT_DYNAMIC;
}

SECTIONS
{
    . = load_addr + SIZEOF_HEADERS;

    kernel_image_start = .;

    .text :
    {
        section_text_start = .;
        *(.text .text.*)
        section_text_end = .;
    } :text

    .rodata ALIGN(CONSTANT(MAXPAGESIZE)) :
    {
        section_rodata_start = .;
        *(.rodata)
        *(.rodata.*)
        KEEP(*(.injected_data))
        section_rodata_end = .;
    } :rodata

    .data ALIGN(CONSTANT(MAXPAGESIZE)) :
    {
        section_data_start = .;
        *(.data)
        *(.data.*)

        KEEP(*(.requests_start_marker))
        KEEP(*(.requests))
        KEEP(*(.requests_end_marker))
        section_data_end = .;
    } :data

    .dynamic : {
        *(.dynamic)
    } :data :dynamic

    .bss ALIGN(CONSTANT(MAXPAGESIZE)) :
    {
        section_bss_start = .;
        *(COMMON)
        *(.bss .bss.*)
        section_bss_end = .;
    } :data

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note .note.*)
        *(.interp)
    }

    kernel_image_end = .;
}
