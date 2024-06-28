from tsp import solve_tsp

from scipy.sparse import csr_matrix
from scipy.sparse.csgraph import construct_dist_matrix, shortest_path
import numpy as np

from itertools import pairwise
from pathlib import Path


class Lambdaman:
    def __init__(self, task_index: int):
        self.task = [
            list(line)
            for line in Path(f'history/lambdaman/{task_index}').read_text().strip().split('\n')
        ]
        self.points = [
            (i, j)
            for i, row in enumerate(self.task)
            for j, value in enumerate(row)
            if value != '#'
        ]

        self.n = len(self.points)

        for point, (i, j) in enumerate(self.points):
            if self.task[i][j] == 'L':
                self.L = point

        self.pos_to_point = {
            point: i
            for i, point in enumerate(self.points)
        }

        self.__calc_adjency_matrix()
        self.__calc_distance_matrix()

    def task_to_text(self):
        return '\n'.join(
            ''.join(
                el
                for el in row
            )
            for row in self.task
        )


    def __calc_adjency_matrix(self):
        adjency_matrix = np.zeros([self.n, self.n], int)
        for i, (pos_0, pos_1) in enumerate(self.points):
            for shift_0, shift_1 in (
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
            ):
                j = self.pos_to_point.get((pos_0 + shift_0, pos_1 + shift_1), None)
                if j is not None:
                    adjency_matrix[i][j] = 1

        self.adjency_matrix = csr_matrix(adjency_matrix)

    def __calc_distance_matrix(self):
        distance_matrix_float, predecessors = shortest_path(self.adjency_matrix, directed=False, return_predecessors=True, method='FW')
        self.distance_matrix_float = [
            [
                int(el)
                for el in row
            ]
            for row in distance_matrix_float
        ]
        self.predecessors = predecessors

    def get_path(self, from_point, to_point) -> list[int]:
        result = [to_point]
        while from_point != self.predecessors[from_point, to_point]:
            to_point = int(self.predecessors[from_point, to_point])
            result.append(to_point)

        result.reverse()

        return result

    def solve(self, search_parameters = None):
        distance_matrix = [
            [
                # skip return to L point
                int(el) if i != self.L else 0
                for i, el in enumerate(row)
            ]
            for row in self.distance_matrix_float
        ]

        tsp_solution, cost = solve_tsp(distance_matrix, self.L, search_parameters)

        assert tsp_solution[0] == self.L

        solution = [self.L] + [
            point
            for from_point, to_point in pairwise(tsp_solution)
            for point in self.get_path(from_point, to_point)
        ]

        return solution

    def solution_to_text(self, solution) -> list[str]:
        result = []
        for from_point, to_point in pairwise(solution):
            (from_0, from_1), (to_0, to_1) = self.points[from_point], self.points[to_point]

            match (to_0 - from_0, to_1 - from_1):
                case -1, 0:
                    result.append('U')
                case 1, 0:
                    result.append('D')
                case 0, -1:
                    result.append('L')
                case 0, 1:
                    result.append('R')
                case _:
                    print(from_0 - to_0, from_1 - to_1)
                    assert False

        return ''.join(result)
