#include <AccelStepper.h>

AccelStepper LeftBackWheel(AccelStepper::DRIVER, 12, 13); // A motor
AccelStepper LeftFrontWheel(AccelStepper::DRIVER, 4, 7); // Z motor
AccelStepper RightBackWheel(AccelStepper::DRIVER, 3, 6); // Y motor
AccelStepper RightFrontWheel(AccelStepper::DRIVER, 2, 5); // X motor

String command = "";
int wheelSpeed = 1000;
const int led = 13;

void setup() {
  LeftFrontWheel.setMaxSpeed(3000);
  LeftBackWheel.setMaxSpeed(3000);
  RightFrontWheel.setMaxSpeed(3000);
  RightBackWheel.setMaxSpeed(3000);

  Serial.begin(115200);
  pinMode(led, OUTPUT);
}

void loop() {
  // Read incoming serial characters
  while (Serial.available()) {
    char c = Serial.read();
    if (c == '\n') {
      executeCommand(command);
      command = "";
    } else {
      command += c;
    }
  }

  // Continuously run steppers
  LeftFrontWheel.runSpeed();
  LeftBackWheel.runSpeed();
  RightFrontWheel.runSpeed();
  RightBackWheel.runSpeed();

  // Monitor battery voltage
  int sensorValue = analogRead(A0);
  float voltage = sensorValue * (5.0 / 1023.0) * 3;
  digitalWrite(led, voltage < 11 ? HIGH : LOW);
}

void executeCommand(String cmd) {
  cmd.trim(); // Remove extra spaces/newlines

  if (cmd == "forward") moveForward();
  else if (cmd == "backward") moveBackward();
  else if (cmd == "left") moveSidewaysLeft();
  else if (cmd == "right") moveSidewaysRight();
  else if (cmd == "rotate_left") rotateLeft();
  else if (cmd == "rotate_right") rotateRight();
  else if (cmd == "stop") stopMoving();
  else if (cmd.startsWith("speed")) {
    int s = cmd.substring(6).toInt();
    wheelSpeed = constrain(s, 100, 3000);
    Serial.print("Speed set to: ");
    Serial.println(wheelSpeed);
  } else {
    Serial.print("Unknown command: ");
    Serial.println(cmd);
  }
}

void moveForward() {
  setAllMotors(wheelSpeed, wheelSpeed, wheelSpeed, wheelSpeed);
}
void moveBackward() {
  setAllMotors(-wheelSpeed, -wheelSpeed, -wheelSpeed, -wheelSpeed);
}
void moveSidewaysRight() {
  setAllMotors(wheelSpeed, -wheelSpeed, -wheelSpeed, wheelSpeed);
}
void moveSidewaysLeft() {
  setAllMotors(-wheelSpeed, wheelSpeed, wheelSpeed, -wheelSpeed);
}
void rotateLeft() {
  setAllMotors(-wheelSpeed, -wheelSpeed, wheelSpeed, wheelSpeed);
}
void rotateRight() {
  setAllMotors(wheelSpeed, wheelSpeed, -wheelSpeed, -wheelSpeed);
}
void stopMoving() {
  setAllMotors(0, 0, 0, 0);
}

void setAllMotors(int lf, int lb, int rf, int rb) {
  LeftFrontWheel.setSpeed(-lf);
  LeftBackWheel.setSpeed(-lb);
  RightFrontWheel.setSpeed(rf);
  RightBackWheel.setSpeed(rb);
}