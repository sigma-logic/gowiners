module top(
    input logic btn,

    output logic led
);

    assign led = ~btn;

endmodule: top
