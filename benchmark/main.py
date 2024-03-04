from time import time
from functools import wraps
from py_rust import calculate_pi

def timeit(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        start = time()
        result = func(*args, **kwargs)
        end = time()
        print(f"{func.__name__} executed in {round((end - start), 2)} seconds.")
        return result
    return wrapper

@timeit
def calculate_pi_with_python(n_terms: int) -> float:
    numerator = 4.0
    denominator = 1.0
    operation = 1.0
    pi = 0.0
    for _ in range(n_terms):
        pi += operation * (numerator / denominator)
        denominator += 2.0
        operation *= -1.0
    return pi

@timeit
def calculate_pi_with_pyo3(n_terms: int) -> float:
    return calculate_pi(n_terms)
    

if __name__ == "__main__":
    N = 100_000_000
    pi = calculate_pi_with_python(N)
    print(f"π = {pi} - Python")
    pi = calculate_pi_with_pyo3(N)
    print(f"π = {pi} - PyO3")
