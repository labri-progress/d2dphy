import struct
import numpy as np
import matplotlib.pyplot as plt

class CSIPacket:
    def __init__(self, num_csi, csis):
        self.num_csi = num_csi
        self.csis = csis

    @classmethod
    def from_bytes(cls, data):
        # Extract the number of CSI elements (u32)
        num_csi, = struct.unpack('<I', data[:4])
        
        # Extract the CSI values (u16) according to the number of CSI elements
        csis = list(struct.unpack('<' + 'H' * num_csi, data[4:4 + num_csi * 2]))
        
        return cls(num_csi, csis)

    def display_as_list(self):
        print(f'Number of CSI: {self.num_csi}')
        print(f'CSI Values: {self.csis}')

    def plot(self, label=None, color=None):
        plt.plot(range(len(self.csis)), self.csis, label=label, color=color)

def parse_csi_file(filename):
    with open(filename, 'rb') as f:
        data = f.read()
    return CSIPacket.from_bytes(data)

def main():
    import argparse

    parser = argparse.ArgumentParser(description="Parse and visualize CSI data.")
    parser.add_argument('file1', type=str, help="First CSI file to parse")
    parser.add_argument('file2', type=str, nargs='?', help="Second CSI file to parse for comparison")
    parser.add_argument('--list', action='store_true', help="Display CSI values as a list")
    parser.add_argument('--plot', action='store_true', default=True, help="Plot the CSI values over time")
    parser.add_argument('-sm', '--substract-mean', action='store_true', help="Substract the mean from the CSI values")
    
    args = parser.parse_args()

    csi_packet1 = parse_csi_file(args.file1)
    if args.file2:
        csi_packet2 = parse_csi_file(args.file2)

    if args.substract_mean:
        m1 = np.mean(csi_packet1.csis)
        csi_packet1.csis = [csi - m1 for csi in csi_packet1.csis]
        if args.file2:
            m2 = np.mean(csi_packet2.csis)
            csi_packet2.csis = [csi - m2 for csi in csi_packet2.csis]

    if args.list:
        csi_packet1.display_as_list()
    elif args.plot:
        csi_packet1.plot(label=args.file1, color='blue')

        if args.file2:
            csi_packet2.plot(label=args.file2, color='red')
        
        plt.xlabel("Sample Index")
        plt.ylabel("CSI Value")
        plt.title("CSI Values Over Time")
        plt.legend()
        plt.show()

if __name__ == "__main__":
    main()

