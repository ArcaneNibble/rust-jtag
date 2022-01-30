from enum import Enum, auto

class JTAGState(Enum):
    TestLogicReset = auto()
    RunTestIdle = auto()

    SelectDR = auto()
    CaptureDR = auto()
    ShiftDR = auto()
    Exit1DR = auto()
    PauseDR = auto()
    Exit2DR = auto()
    UpdateDR = auto()

    SelectIR = auto()
    CaptureIR = auto()
    ShiftIR = auto()
    Exit1IR = auto()
    PauseIR = auto()
    Exit2IR = auto()
    UpdateIR = auto()

STATE_TRANSITIONS = {
    #                            TMS = 0                    TMS = 1
    JTAGState.TestLogicReset:   (JTAGState.RunTestIdle,     JTAGState.TestLogicReset),
    JTAGState.RunTestIdle:      (JTAGState.RunTestIdle,     JTAGState.SelectDR),

    JTAGState.SelectDR:         (JTAGState.CaptureDR,       JTAGState.SelectIR),
    JTAGState.CaptureDR:        (JTAGState.ShiftDR,         JTAGState.Exit1DR),
    JTAGState.ShiftDR:          (JTAGState.ShiftDR,         JTAGState.Exit1DR),
    JTAGState.Exit1DR:          (JTAGState.PauseDR,         JTAGState.UpdateDR),
    JTAGState.PauseDR:          (JTAGState.PauseDR,         JTAGState.Exit2DR),
    JTAGState.Exit2DR:          (JTAGState.ShiftDR,         JTAGState.UpdateDR),
    JTAGState.UpdateDR:         (JTAGState.RunTestIdle,     JTAGState.SelectDR),

    JTAGState.SelectIR:         (JTAGState.CaptureIR,       JTAGState.TestLogicReset),
    JTAGState.CaptureIR:        (JTAGState.ShiftIR,         JTAGState.Exit1IR),
    JTAGState.ShiftIR:          (JTAGState.ShiftIR,         JTAGState.Exit1IR),
    JTAGState.Exit1IR:          (JTAGState.PauseIR,         JTAGState.UpdateIR),
    JTAGState.PauseIR:          (JTAGState.PauseIR,         JTAGState.Exit2IR),
    JTAGState.Exit2IR:          (JTAGState.ShiftIR,         JTAGState.UpdateIR),
    JTAGState.UpdateIR:         (JTAGState.RunTestIdle,     JTAGState.SelectDR),
}

def search(start, end):
    if start == end:
        return []

    q = [(start, [])]
    # print(q)

    while q:
        (state, path) = q.pop(0)

        c0 = STATE_TRANSITIONS[state][0]
        c1 = STATE_TRANSITIONS[state][1]
        # print(state, c0, c1)

        if c0 == end:
            return path + [False]
        if c1 == end:
            return path + [True]

        q.append((c0, path + [False]))
        q.append((c1, path + [True]))
        # print(q)
print("pub const fn jtag_transition(start: JTAGState, end: JTAGState) -> &'static [bool] {")
print("match start {")

for start in JTAGState:
    print(f"JTAGState::{start._name_} => {{")
    print("match end {")
    for end in JTAGState:
        print(f"JTAGState::{end._name_} => {{")
        path = search(start, end)
        # print(start, end, path)
        print("&[")
        for pathelem in path:
            if pathelem:
                print("true,")
            else:
                print("false,")
        print("]")
        print("}")
    print("}")
    print("}")

print("}")
print("}")
