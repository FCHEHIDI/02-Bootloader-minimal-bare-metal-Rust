const RCC_AHB1ENR: *mut u32 = 0x4002_3830 as *mut u32;
const RCC_APB1ENR: *mut u32 = 0x4002_3840 as *mut u32;
const GPIOA_MODER: *mut u32 = 0x4002_0000 as *mut u32;
const GPIOA_AFRL:  *mut u32 = 0x4002_0020 as *mut u32;
const USART2_SR:  *const u32 = 0x4000_4400 as *const u32;
const USART2_DR:  *mut u32   = 0x4000_4404 as *mut u32;
const USART2_BRR: *mut u32   = 0x4000_4408 as *mut u32;
const USART2_CR1: *mut u32   = 0x4000_440C as *mut u32;

/// Initialise USART2 à 115200 baud (HSI 16 MHz)
pub fn uart_init() {
    unsafe {
    // 1. Activer les horloges pour GPIOA et USART2
    let ahb1 = core::ptr::read_volatile(RCC_AHB1ENR);
    core::ptr::write_volatile(RCC_AHB1ENR, ahb1 | (1 << 0)); // GPIOA
    let apb1 = core::ptr::read_volatile(RCC_APB1ENR);
    core::ptr::write_volatile(RCC_APB1ENR, apb1 | (1 << 17)); // USART2

    // 2. PA2 et PA3 en mode AF (0b10)
    let moder = core::ptr::read_volatile(GPIOA_MODER);
    let moder = (moder & !(0b1111 << (4))) | (0b1010 << (4)); // PA2 et PA3
    core::ptr::write_volatile(GPIOA_MODER, moder);

    // 3. PA2 et PA3 -> AF7 (USART2)
    let afrl = core::ptr::read_volatile(GPIOA_AFRL);
    let afrl = (afrl & !(0xFFFF << 8)) | (0x7777 << 8);
    core::ptr::write_volatile(GPIOA_AFRL, afrl);

    // 4. BRR : 16 MHz / 115200
    core::ptr::write_volatile(USART2_BRR, 16_000_000 / 115_200);

    // 5. Activer USART2 : UE + TE +RE (bits 13, 3, 2)
    core::ptr::write_volatile(USART2_CR1, (1 << 13) | (1 << 3) | (1 << 2));
    }
}

/// Envoie un octet (bloquante)
pub fn uart_putc(byte: u8) {
    unsafe {
        // Attendre que le buffer de transmission soit vide
        while core::ptr::read_volatile(USART2_SR) & (1 << 7) == 0 {}
        // Écrire l'octet à transmettre
        core::ptr::write_volatile(USART2_DR, byte as u32);
    }
}

/// Reçoit un octet (bloquant)
pub fn uart_getc() -> u8 {
    unsafe {
        // Attendre que des données soient disponibles
        while core::ptr::read_volatile(USART2_SR) & (1 << 5) == 0 {}
        // Lire et retourner l'octet reçu
        core::ptr::read_volatile(USART2_DR) as u8
    }
}

/// Envoie un slice d'octets 
pub fn uart_write(data: &[u8]) {
    for &byte in data {
        uart_putc(byte);
    }
}