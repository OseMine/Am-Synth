import numpy as np

def moog_filter(I, fc, Q):
    # Berechnung der Moog-Charakteristik
    omega = 2 * np.pi * fc
    return I * (-4 * np.pi ** 2 * fc ** 2 + 1) / (1 + Q * omega * 1j)

def roland_filter(I, fc, Q):
    # Berechnung der Roland-Charakteristik
    omega = 2 * np.pi * fc
    return (2 * np.pi * I * fc * 1j) / (-4 * np.pi ** 2 * fc ** 2 + 1 + Q * omega * 1j)

def filter_output(I, fc, Q, filter_type="Moog"):
    if filter_type == "Moog":
        return moog_filter(I, fc, Q)
    elif filter_type == "Roland":
        return roland_filter(I, fc, Q)
    else:
        raise ValueError("Invalid filter type. Choose 'Moog' or 'Roland'.")

# Beispielhafte Parameter
I = 1.0  # Beispiel-Eingangssignal
fc = 1000  # Cutoff-Frequenz in Hz
Q = 0.7   # Beispielhafter Gütefaktor

# Schalterstellung abfragen
filter_type = "Roland"  # Oder "Roland" je nach Schalterstellung

# Berechnung der Filterausgabe
output_signal = filter_output(I, fc, Q, filter_type=filter_type)
print(f"Ausgangssignal für {filter_type} Charakteristik: {output_signal}")
