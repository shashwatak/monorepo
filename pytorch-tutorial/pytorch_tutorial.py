import torch
import numpy as np

# We move our tensor to the current accelerator if available
if torch.cuda.is_available():
    device = torch.device("cuda")
    print("Cuda Accel")
    torch.set_default_device(device)


data = [[1, 2],[3, 4]]
x_data = torch.tensor(data)

x_ones = torch.ones_like(x_data) # retains the properties of x_data
print(f"Ones Tensor: \n {x_ones} \n")

x_rand = torch.rand_like(x_data, dtype=torch.float) # overrides the datatype of x_data
print(f"Random Tensor: \n {x_rand} \n")


shape = (2,3,)
rand_tensor = torch.rand(shape)
ones_tensor = torch.ones(shape)
zeros_tensor = torch.zeros(shape)

print(f"Random Tensor: \n {rand_tensor} \n")
print(f"Ones Tensor: \n {ones_tensor} \n")
print(f"Zeros Tensor: \n {zeros_tensor}")

# tensor = torch.rand(3,4)
tensor = torch.rand(3,4)
tensor.to("cuda")

print(f"Shape of tensor: {tensor.shape}")
print(f"Datatype of tensor: {tensor.dtype}")
print(f"Device tensor is stored on: {tensor.device}")


print(f"try slice")
print(tensor)
tensor[:,1] = 0
print(tensor)

print(f"try cat d1")
t1 = torch.cat([tensor, tensor, tensor], dim=1)
print(f"Shape of tensor: {t1.shape}")
print(f"Datatype of tensor: {t1.dtype}")
print(f"Device tensor is stored on: {t1.device}")
print(t1)


print(f"try cat d2")
t2 = torch.cat([tensor, tensor, tensor], dim=-2)
print(f"Shape of tensor: {t2.shape}")
print(f"Datatype of tensor: {t2.dtype}")
print(f"Device tensor is stored on: {t2.device}")
print(t2)


print(f"try stack d1")
t1 = torch.stack([tensor, tensor, tensor], dim=1)
print(f"Shape of tensor: {t1.shape}")
print(f"Datatype of tensor: {t1.dtype}")
print(f"Device tensor is stored on: {t1.device}")
print(t1)


print(f"try stack d2")
t2 = torch.stack([tensor, tensor, tensor], dim=-2)
print(f"Shape of tensor: {t2.shape}")
print(f"Datatype of tensor: {t2.dtype}")
print(f"Device tensor is stored on: {t2.device}")
print(t2)


# This computes the element-wise product
print(f"tensor.mul(tensor) \n {tensor.mul(tensor)} \n")
# Alternative syntax:
print(f"tensor * tensor \n {tensor * tensor}")

# in place
print(tensor, "\n")
tensor.add_(5)
print(tensor)

