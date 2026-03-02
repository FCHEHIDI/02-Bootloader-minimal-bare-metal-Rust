#!/usr/bin/env python3
"""
flash_tool.py -- Envoie une image binaire au bootloader via UART
Usage: python flash_tool.py <port> <image.bin>
Exemple: python flash_tool.py COM3 app-test.bin
"""

import sys
import struct
import serial # pip install pyserial

def flash(port: str, image_path: str) -> None:
    with open(image_path, "rb") as f:
        data = f.read()

    size = len(data)
    print(f"[*] Image: {image_path} ({size} octects)")
    
    with serial.Serial(port, baudrate=115200, timeout=5) as ser:
        print(f"[*] Connexion à {port}...")

        # 1. Envoie la taille sur 4 octets big-endian
        ser.write(struct.pack(">I", size))

        # 2. Envoyer l'image
        ser.write(data)
        print(f"[*] Image envoyée, attente réponse...")

        # 3. Lire la réponse du bootloader
        response = ser.readline().decode("utf-8", errors="ignore").strip()
        if response.startswith("K"):
            print("[OK] Flash réussi")
        else:
            print(f"[ERREUR] Réponse inattendue : {response!r}")
            sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python flash_tool.py <port> <image.bin>")
        sys.exit(1)
    flash(sys.argv[1], sys.argv[2])