OUTPUT_FORMAT(elf64-x86-64)
ENTRY(_start)
PHDRS
{
    text   PT_LOAD FLAGS(5);
    rodata PT_LOAD FLAGS(4);
    data   PT_LOAD FLAGS(6);
}
SECTIONS
{
    . = 0xffffffff80000000;
    .text : ALIGN(4K) {
        *(.text .text.*)
    } :text
    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    } :rodata
    .requests : ALIGN(4K) {
        *(.requests .requests.*)
    } :rodata
    .data : ALIGN(4K) {
        *(.data .data.*)
    } :data
    .bss : ALIGN(4K) {
        *(COMMON)
        *(.bss .bss.*)
    } :data
    /DISCARD/ : {
        *(.eh_frame)
        *(.note .note.*)
    }
}