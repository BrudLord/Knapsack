### Описание

Проект содержит инстументы для решения задачи о бинарном рюкзаке (0-1 Knapsack Problem) с использованием различных алгоритмов. Кроме того, библиотека включает экспериментальную среду, которая позволяет сравнивать алгоритмы по времени выполенения  на рандомизированных наборах данных.

---

### Формулировка задачи о рюкзаке

Дано N предметов, где $n_i$-й предмет имеет:
- массу $w_i$ > 0 
- стоимость $p_i$ > 0  

Необходимо выбрать из этих предметов такой набор, чтобы:
1. Суммарная масса не превосходила заданной величины W (вместимость рюкзака).
2. Суммарная стоимость была максимальна.

---

### Алгоритмы решения 0-1 Knapsack Problem

1. Полный перебор через битовые маски 
2. Полный перебор через рекурсию  
3. Meet in the middle (not implemented)
4. Динамическое программирование `O(nW)`  
5. Ленивая динамика  
6. Жадный алгоритм  
7. FPTAS (Fully Polynomial-Time Approximation Scheme) (not implemented)
8. Метод ветвей и границ (not implemented)

---

### Сборка проекта
- Для сборки библиотеки смотрите [`knapsack_library/README.md`](knapsack_library/README.md)
- Для сборки тестовой среды смотрите [`experimentator/README.md`](experimentator/README.md)

---

### Contributing

См. файл [`CONTRIBUTING.md`](CONTRIBUTING.md) для получения дополнительной информации.

---

### License

Проект распространяется под лицензией MIT. См. файл [`LICENSE`](LICENSE) для получения дополнительной информации.

---

### Авторы

- Акимов Евгений (lottery7)
- Кравцова Екатерина (bdmpea)
- Шевченко Будимир (BrudLord)



