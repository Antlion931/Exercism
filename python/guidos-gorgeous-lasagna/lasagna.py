"""Functions used in preparing Guido's gorgeous lasagna.

Learn about Guido, the creator of the Python language:
https://en.wikipedia.org/wiki/Guido_van_Rossum

This is a module docstring, used to describe the functionality
of a module and its functions and/or classes.
"""


EXPECTED_BAKE_TIME = 40


def bake_time_remaining(elapsed_bake_time):
    """Calculate the bake time remaining.

    :param elapsed_bake_time: int - baking time already elapsed.
    :return: int - remaining bake time (in minutes) derived from 'EXPECTED_BAKE_TIME'.

    Function that takes the actual minutes the lasagna has been in the oven as
    an argument and returns how many minutes the lasagna still needs to bake
    based on the `EXPECTED_BAKE_TIME`.
    """

    return EXPECTED_BAKE_TIME - max(elapsed_bake_time, 0)

PREPARATION_TIME = 2

def preparation_time_in_minutes(number_of_layers):
    """Calculate the preparation time.

    :param number_of_layers: int - number of layers in the lasagna.
    :return: int - total preparation time (in minutes) derived from 'PREPARATION_TIME'.

    Function that takes the number of layers in the lasagna as an argument and returns
    how many minutes it will take to prepare the lasagna based on the `PREPARATION_TIME`.
    """

    return max(number_of_layers, 0) * PREPARATION_TIME



def elapsed_time_in_minutes(number_of_layers, elapsed_bake_time):
    """Calculate the elasped time

    :param number_of_layers: int - number of layers in the lasagna.
    :param elapsed_bake_time: int - minutes a lasagna was in the oven.
    :return: int - minutes it took up to this point to make a lasagna.

    Function that takes the number of layers and time spend in the oven as an argument and returns
    how many minutes it took to this point to make lasagna.
    """

    return preparation_time_in_minutes(number_of_layers) + max(elapsed_bake_time, 0)
