use core::arch::asm;

/// Adresse du registre VTOR (Vector Table Offset Register)
const VTOR: *mut u32 = 0xE000_ED08 as *mut u32;

/// Saute à l'application située à `app_base`.
///
/// # Safety
/// - `app_base` doit pointer vers une image valide avec une table de vecteurs correctes
/// - Appelé uniquement après vérification CRC de l'image
pub unsafe fn jump_to_app(app_base: u32) -> ! {
    // 1. Repositionner VTOR vers la table des vecteurs de l'app
    core::ptr::write_volatile(VTOR, app_base);

    // 2. Lire le MSP initial de l'app (premier u32 de la table des vecteurs)
    let msp = core::ptr::read_volatile(app_base as *const u32);

    // 3. Lire l'adresse du Reset_Handler (deuxième u32)
    let reset_handler = core::ptr::read_volatile((app_base + 4) as *const u32);

    // 4. Configurer le MSP et brancher, impossible sans assembleur inline
    asm!(
        "msr msp, {msp}", // écrire le nouveau stack pointer
        "bx {reset}",    // brancher vers le Reset_Handler
        msp = in(reg) msp,
        reset = in(reg) reset_handler,
        options(noreturn),
    );
}