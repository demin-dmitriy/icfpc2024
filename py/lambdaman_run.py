from lambdaman_solver import Lambdaman

from ortools.constraint_solver import routing_enums_pb2
from ortools.constraint_solver import pywrapcp

from pathlib import Path
from pathlib import Path
from time import sleep

import com


def solve_lambdaman(i, search_parameters=None):
    search_parameters = pywrapcp.DefaultRoutingSearchParameters()
    search_parameters.first_solution_strategy = (
        routing_enums_pb2.FirstSolutionStrategy.PATH_CHEAPEST_ARC
    )

    search_parameters.local_search_metaheuristic = (routing_enums_pb2.LocalSearchMetaheuristic.GUIDED_LOCAL_SEARCH)
    search_parameters.time_limit.seconds = 60 * 60
    search_parameters.log_search = True

    print(f'\n\nLambdaman #{i}')
    destination_dir = Path(f'solutions/lambdaman')
    destination_dir.mkdir(exist_ok=True, parents=True)

    lambdaman = Lambdaman(i)

    print(f'\nTask:\n\n{lambdaman.task_to_text()}')


    solution = lambdaman.solve(search_parameters)
    solution_text = lambdaman.solution_to_text(solution)

    print(f'\nSoluton: \n{solution} \n{solution_text}')

    destination = destination_dir / f'{i}_{len(solution_text)}'
    if destination.exists():
        print(f'{destination} exists')
        return

    destination.write_text(solution_text)



from multiprocessing import Pool

with Pool(18) as p:
        results_iter = p.starmap(
            solve_lambdaman,
            (
                (i,)
                for i in [21]
                #if i not in [6, 9, 10, 20, 21]
            )
        )
