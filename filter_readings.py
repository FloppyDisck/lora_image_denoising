import numpy as np

is_clean = []
rss = []
snr = []
fe = []

with open('drive1.txt', 'r') as file:
    for line in file:
        if line != "":
            if line[:9] == "message: ":
                message = line[9:][:-1].split(", ")[0]

                if message[:11] == "{\"message\":" and message[15:] == "}" and message[11:15].isnumeric():
                    is_clean.append(1)
                else:
                    is_clean.append(0)
            elif line[:6] == "RSSI: ":
                rss.append(int(line[6:]))
            elif line[:5] == "SNR: ":
                snr.append(int(line[5:]))
            elif line[:4] == "FE: ":
                fe.append(int(line[4:]))

print(len(is_clean))
print(len(rss))
print(len(snr))
print(len(fe))

full = []
for i in range(len(is_clean)):
    full.append([is_clean[i], rss[i], snr[i], fe[i]])

full = np.array(full)
print(full)
np.save("data/is_noisy.npy", full)