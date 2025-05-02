# Rust PPM Compression

This project was outlined and implemented as an assignment project in Noah Daniels' CSC 411 coursework in Computer Organization. 

The rpeg crate provides a module in which compression and decompression are implemented for the PPM image file type. It accomplishes this by using matrix arithmetic to convert from RGB values to component video code format, and compressing through encoding the values in bits, and decompressing by decoding the bits back to RGB values.

## Team Members
- Aeyva Rebelo 

## Acknowledgements
We would like to acknowledge the following individuals for their help and collaboration:
- Alex Bergeron
    - Originally, this project was attempted, and thus, the basic outline and some functions were borrowed from the collaborative efforts put forth. Much of the project has been reworked, but alas, this was completed by building upon the efforts from both parties.
- Bitpack Library
    - The Bitpack library was used to help with the encoding and decoding of the data, as it provided functions to pack and unpack encoded bits  which was necessary for the compression and decompression of the image. Thanks for the development of this library go to Kai Maffucci and Alex Bergeron, as this form of bitpack was used during the original development effort between Alex and I. 
- csc411_image Library
    - The csc411_image library was used to help with the reading and writing of the PPM image files, as it provided the necessary structs and functions to work with images of PPM format. 
- Copilot AI
    - Copilot AI was used to help with the implementation of the project, as it helped with the syntax and structure of the code, as well as providing some ideas for the implementation of the project, and finishing documentation.

## Usage

- Simply run ```cargo build``` in the parent workspace directory, and utilize the produced ```rpeg``` binary in the ```./target``` directory.
- ```rpeg [flags] [filename]```
    - [flags]
        - -d: Decompress procedure.
        - -c: Compression procedure.
        - -r: Round-trip, compress and decompress the file.
    - [filename]
        - The name of the file to be decompressed, if using a PPM file to be compressed with the -c flag, or a compressed image file to be decompressed with the -d flag.  
        - If no filename is provided, the program will read from standard input.
            - The round trip flag will not work with standard input, as the program will not be able to read the compressed file from standard input.
- **It is important to note that the compression will be performed *in-place,* meaning that the file will be overwritten in compression.**



## Architecture
Our solution follows the following architecture:
- `main.rs`: The main function, which parses the command line arguments and calls the appropriate functions.
- `codec.rs`: Functions that are used in the compression / decompression process of the image, called by the input flags.
- `image.rs`: Contains the necessary functions for formatting and taking in the data and dimensions of given input when dealing with compressed files.
- `rgb_cvc.rs`: Implementations for converting the RGB values to floating points, which is necessary for calculating the component video code format values. Also contains the reversed process.
- `cvc_blocks.rs`: Contains the struct and functions for the blocks of component video code format values and subdivides the image's data in CVC data into 2x2 blocks, which effectively compresses the data. Also implements the reverse subdivision process. 
- `blocks_bwc.rs`: Implements a tuple struct for the round-trip encoding of the blocks of CVC data into bits, which are packed into a 32-bit codeword according to the algorithm's specifications. Also implements the reverse process of decoding the bits back into the blocks of CVC data.
- `bitpack.rs`: Contains the functions for packing the passed values into a bit code word.
- `lib.rs`: Contains the necessary imports for the modules and functions to be used throughout the program's modules.

### Compression

1. The program reads in the PPM image file, and converts the RGB values to floating point values, which are necessary for the calculations of the component video code format values.
2. Using the floating point RGB values, the program converts them into component video code format values, which are then subdivided into 2x2 blocks. 
3. With matrix arithmetic, the component video code values, `Y`, `Pb`, and `Pr`, are used in the calculation of the `a`, `b`, `c`, and `d` values, as well as `PbAvg` and `PrAvg`, which are used in the encoding of the data into bits in the following format: 

| Value Type            | Width | LSB |
|-----------------------|-------|-----|
| a (Unsigned scaled integer) | 9 bits | 23  |
| b (Signed scaled integer)   | 5 bits | 18  |
| c (Signed scaled integer)   | 5 bits | 13  |
| d (Signed scaled integer)   | 5 bits | 8   |
| index(PB) (Unsigned index)  | 4 bits | 4   |
| index(PR) (Unsigned index)  | 4 bits | 0   | 

4. The values are then packed into a 32-bit codeword, which is then written to the output file.

### Decompression

1. The program reads in the compressed file, and unpacks the 32-bit codewords into the values for `a`, `b`, `c`, `d`, `index(PB)`, and `index(PR)`, and the indeces are converted back into `Pb` and `Pr` values.
    - Because we had averaged the Pb and Pr values, and they will be redistributed to each pixel in the 2x2 block, rather than the original 2x2 block's values, we have approximated and compressed the data.  
2. The values are then used to calculate the component video code format values of each 2x2 blocks' pixels, which are then un-subdivided into the original image's pixel format.
3. The component video code format values are then converted back into RGB floating point values, which are then cast into integers.
4. The integer RGB values are then written to the output file.

## Time Spent
As this is not the first attempt of the project, I'll go ahead and summarize it for this attempt of the implementation.

- Hours spent analyzing the problems: 4 hours
- Hours spent solving the problems: 15 hours

### Design Documentation

For full implementation details, design rationale, and project planning, see [design.pdf](./design.pdf).
