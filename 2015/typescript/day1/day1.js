"use strict";
var __values = (this && this.__values) || function(o) {
    var s = typeof Symbol === "function" && Symbol.iterator, m = s && o[s], i = 0;
    if (m) return m.call(o);
    if (o && typeof o.length === "number") return {
        next: function () {
            if (o && i >= o.length) o = void 0;
            return { value: o && o[i++], done: !o };
        }
    };
    throw new TypeError(s ? "Object is not iterable." : "Symbol.iterator is not defined.");
};
var __read = (this && this.__read) || function (o, n) {
    var m = typeof Symbol === "function" && o[Symbol.iterator];
    if (!m) return o;
    var i = m.call(o), r, ar = [], e;
    try {
        while ((n === void 0 || n-- > 0) && !(r = i.next()).done) ar.push(r.value);
    }
    catch (error) { e = { error: error }; }
    finally {
        try {
            if (r && !r.done && (m = i["return"])) m.call(i);
        }
        finally { if (e) throw e.error; }
    }
    return ar;
};
exports.__esModule = true;
var fs = require("fs");
function readFile(fileName) {
    var data = fs.readFileSync(fileName);
    return data.toString().split("");
}
function part1() {
    var inputs = readFile("../../inputs/input.txt");
    var up = 0;
    var down = 0;
    inputs.forEach(function (item, _) {
        if (item === ")") {
            up += 1;
        }
        else if (item === "(") {
            down += 1;
        }
    });
    return down - up;
}
function part2() {
    var e_1, _a;
    var inputs = readFile("../../inputs/input.txt");
    var position = 0;
    try {
        for (var _b = __values(inputs.entries()), _c = _b.next(); !_c.done; _c = _b.next()) {
            var _d = __read(_c.value, 2), index = _d[0], value = _d[1];
            if (value === ")") {
                position -= 1;
            }
            if (value === "(") {
                position += 1;
            }
            if (position === -1) {
                return index + 1;
            }
        }
    }
    catch (e_1_1) { e_1 = { error: e_1_1 }; }
    finally {
        try {
            if (_c && !_c.done && (_a = _b["return"])) _a.call(_b);
        }
        finally { if (e_1) throw e_1.error; }
    }
    return -1;
}
// console.log(part1());
console.log(part2());
