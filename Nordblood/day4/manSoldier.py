class SleepingPeople(object):
    """docstring"""

    def __init__(self, id, ListMinute, ):
        """Constructor"""
        self.id = id
        self.ListMinute = ListMinute

    def minuteOfSleep(self):
        for i in self.ListMinute:

        return

    def drive(self):
        """
        Drive the car
        """
        return "I'm driving a %s %s!" % (self.color, self.vtype)


if __name__ == "__main__":
    car = Vehicle("blue", 5, 4, "car")
    print(car.brake())
    print(car.drive())

    truck = Vehicle("red", 3, 6, "truck")
    print(truck.drive())
    print(truck.brake())
