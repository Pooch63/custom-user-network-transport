def int_to_bytes_string(n):
    byte_list = []
    while n > 0:
        byte_list.append(n % 256)  # Get the least significant byte
        n //= 256  # Move to the next byte
    output = ""
    for byte in bytes(byte_list[::-1]): output += str(byte) # Convert to a string
    return output

print(int_to_bytes_string(1234832057328974389573489573489573489750319849032848923758324))