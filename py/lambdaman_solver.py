from tsp import solve_tsp

from scipy.sparse import csr_matrix
from scipy.sparse.csgraph import construct_dist_matrix, shortest_path
import numpy as np
import time

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

    def get_distance_matrix(self):
        if not hasattr(self, 'distance_matrix'):
            self.__calc_adjency_matrix()
            self.__calc_distance_matrix()

        return self.distance_matrix

    def get_predecessors(self):
        if not hasattr(self, 'distance_matrix'):
            self.__calc_adjency_matrix()
            self.__calc_distance_matrix()

        return self.predecessors

    def task_to_text(self, task = None):
        return '\n'.join(
            ''.join(
                el
                for el in row
            )
            for row in (self.task if task is None else task)
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
        distance_matrix, predecessors = shortest_path(self.adjency_matrix, directed=False, return_predecessors=True, method='FW')
        self.distance_matrix = [
            [
                int(el)
                for el in row
            ]
            for row in distance_matrix
        ]
        self.predecessors = predecessors


    def get_path(self, from_point, to_point) -> list[int]:
        predecessors = self.get_predecessors()
        result = [to_point]
        while from_point != predecessors[from_point, to_point]:
            to_point = int(predecessors[from_point, to_point])
            result.append(to_point)

        result.reverse()

        return result

    def get_tsp_path(self, search_parameters = None):
        distance_matrix = [
            [
                # skip return to L point
                el if i != self.L else 0
                for i, el in enumerate(row)
            ]
            for row in self.get_distance_matrix()
        ]

        tsp_solution, cost = solve_tsp(distance_matrix, self.L, search_parameters)

        assert tsp_solution[0] == self.L

        solution = [self.L] + [
            point
            for from_point, to_point in pairwise(tsp_solution)
            for point in self.get_path(from_point, to_point)
        ]

        return solution

    def solve_tsp(self, search_parameters = None):
        path = self.get_tsp_path()
        return self.points_to_solution(path)


    def points_to_solution(self, points: list[int]) -> list[str]:
        assert points[0] == self.L
        result = []
        for from_point, to_point in pairwise(points):
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

    def apply_solution(self, solution: str) -> list[list[str]]:
        result = [
            [
                el
                for el in row
            ]
            for row in self.task
        ]

        l = self.points[self.L]

        for next_move in solution:
            match next_move:
                case 'R':
                    next_pos = (l[0], l[1] + 1)
                case 'L':
                    next_pos = (l[0], l[1] - 1)
                case 'U':
                    next_pos = (l[0] - 1, l[1])
                case 'D':
                    next_pos = (l[0] + 1, l[1])
                case _:
                    raise(Exception())

            if next_pos in self.pos_to_point:
                result[l[0]][l[1]] = ' '
                l = next_pos
                result[l[0]][l[1]] = 'L'

        return result

    def is_finished_task(self, task):
        return all(
            value != '.'
            for row in task
            for value in row
        )

    def is_valid_solution(self, solution: str) -> bool:
        field = self.apply_solution(solution)

        return self.is_finished_task(field)

    def animate_solution(self, solution: str, destination: Path = Path('solution.txt'), sleep: float = 0.3):
        result = [
            [
                el
                for el in row
            ]
            for row in self.task
        ]

        l = self.points[self.L]

        for next_move in solution:
            match next_move:
                case 'R':
                    next_pos = (l[0], l[1] + 1)
                case 'L':
                    next_pos = (l[0], l[1] - 1)
                case 'U':
                    next_pos = (l[0] - 1, l[1])
                case 'D':
                    next_pos = (l[0] + 1, l[1])
                case _:
                    raise(Exception())

            if next_pos in self.pos_to_point:
                result[l[0]][l[1]] = ' '
                l = next_pos
                result[l[0]][l[1]] = 'L'

                destination.write_text(self.task_to_text(result))
                time.sleep(sleep)
