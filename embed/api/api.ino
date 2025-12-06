const int LED = 6;

struct State {
  int lower;
  int higher;
};

void setup();
void handle_commands();
void setoff_alarm();
void reset_state();
State get_state();

void setup() {
  Serial.begin(9600);
  pinMode(LED, OUTPUT);
}

void handle_commands() {
  while (Serial.available()) {
    int command = Serial.read();
    if (command == 0xAA) {
      State s = get_state();
      Serial.write(s.lower);
      Serial.write(s.higher);

      Serial.flush();
    } else if (command == 0x55) {
      reset_state();
    } else if (command == 0xA5) {
      setoff_alarm();
    }
  }
}

void loop() {
  handle_commands();
}

State get_state() {
  return {12, 13};
}

void setoff_alarm() {
  digitalWrite(LED, HIGH);
}

void reset_state() {
  digitalWrite(LED, LOW);
}
